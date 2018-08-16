class CreateForumTags < ActiveRecord::Migration[5.2]
  def change
    create_table :forum_tags do |t|
      t.string :icon, null: false, limit: 32
      t.string :color, null: false, limit: 16
      t.string :name, null: false, limit: 255
      t.timestamps
    end
    add_index :forum_tags, :name, unique: true
  end
end
